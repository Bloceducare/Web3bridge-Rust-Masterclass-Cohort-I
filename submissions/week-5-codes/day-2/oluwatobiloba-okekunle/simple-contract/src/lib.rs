
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol_storage! {
    pub struct Todo {
        uint256 id;
        string text;
        bool completed;
        address creator;
        uint256 created_at;
    }
}

sol_storage! {
    #[entrypoint]
    pub struct TodoContract {
        mapping(uint256 => Todo) todos;
        uint256 next_id;
        uint256 total_todos;
        address owner;
    }
}

sol! {
    #[derive(Debug)]
    error TodoNotFoundError();
    
    #[derive(Debug)]
    error NotOwnerError();
    
    #[derive(Debug)]
    error AlreadyCompletedError();
    
    #[derive(Debug)]
    error EmptyTextError();
    
    #[derive(Debug)]
    error UnauthorizedError();

    event TodoCreated(uint256 indexed id, string text, address indexed creator);
    event TodoCompleted(uint256 indexed id, address indexed completer);
    event TodoDeleted(uint256 indexed id, address indexed deleter);
    event TodoUpdated(uint256 indexed id, string new_text, address indexed updater);
}

#[derive(SolidityError, Debug)]
pub enum TodoError {
    TodoNotFoundError(TodoNotFoundError),
    NotOwnerError(NotOwnerError),
    AlreadyCompletedError(AlreadyCompletedError),
    EmptyTextError(EmptyTextError),
    UnauthorizedError(UnauthorizedError),
}

#[public]
impl TodoContract {
    #[constructor]
    pub fn constructor(&mut self) {
        self.owner.set(self.vm().msg_sender());
        self.next_id.set(U256::from(1));
        self.total_todos.set(U256::ZERO);
    }

    pub fn create_todo(&mut self, text: String) -> Result<U256, TodoError> {
        if text.is_empty() {
            return Err(TodoError::EmptyTextError(EmptyTextError{}));
        }

        let id = self.next_id.get();
        let creator = self.vm().msg_sender();
        let created_at = U256::from(self.vm().block_timestamp());

        let mut todo = self.todos.setter(id);
        todo.id.set(id);
        todo.text.set_str(&text);
        todo.completed.set(false);
        todo.creator.set(creator);
        todo.created_at.set(created_at);

        self.next_id.set(id + U256::from(1));
        self.total_todos.set(self.total_todos.get() + U256::from(1));

        log(self.vm(), TodoCreated {
            id,
            text,
            creator,
        });

        Ok(id)
    }

    pub fn get_todo(&self, id: U256) -> Result<(U256, String, bool, Address, U256), TodoError> {
        let todo = self.todos.get(id);
        
        if todo.id.get() == U256::ZERO && id != U256::ZERO {
            return Err(TodoError::TodoNotFoundError(TodoNotFoundError{}));
        }

        Ok((
            todo.id.get(),
            todo.text.get_string(),
            todo.completed.get(),
            todo.creator.get(),
            todo.created_at.get(),
        ))
    }

    pub fn complete_todo(&mut self, id: U256) -> Result<(), TodoError> {
        let sender = self.vm().msg_sender();
        let owner = self.owner.get();
        
        let mut todo = self.todos.setter(id);
        
        if todo.id.get() == U256::ZERO && id != U256::ZERO {
            return Err(TodoError::TodoNotFoundError(TodoNotFoundError{}));
        }

        if todo.completed.get() {
            return Err(TodoError::AlreadyCompletedError(AlreadyCompletedError{}));
        }

        if sender != todo.creator.get() && sender != owner {
            return Err(TodoError::UnauthorizedError(UnauthorizedError{}));
        }

        todo.completed.set(true);


        log(self.vm(), TodoCompleted {
            id,
            completer: sender,
        });

        Ok(())
    }

    pub fn update_todo(&mut self, id: U256, new_text: String) -> Result<(), TodoError> {
        if new_text.is_empty() {
            return Err(TodoError::EmptyTextError(EmptyTextError{}));
        }

        let sender = self.vm().msg_sender();
        let owner = self.owner.get();
        
        let mut todo = self.todos.setter(id);
        
        if todo.id.get() == U256::ZERO && id != U256::ZERO {
            return Err(TodoError::TodoNotFoundError(TodoNotFoundError{}));
        }

        if sender != todo.creator.get() && sender != owner {
            return Err(TodoError::UnauthorizedError(UnauthorizedError{}));
        }

        todo.text.set_str(&new_text);


        log(self.vm(), TodoUpdated {
            id,
            new_text,
            updater: sender,
        });

        Ok(())
    }

    pub fn delete_todo(&mut self, id: U256) -> Result<(), TodoError> {
        let todo = self.todos.get(id);
        
        if todo.id.get() == U256::ZERO && id != U256::ZERO {
            return Err(TodoError::TodoNotFoundError(TodoNotFoundError{}));
        }

        let sender = self.vm().msg_sender();
        if sender != todo.creator.get() && sender != self.owner.get() {
            return Err(TodoError::UnauthorizedError(UnauthorizedError{}));
        }

        let mut todo_setter = self.todos.setter(id);
        todo_setter.id.set(U256::ZERO);
        todo_setter.text.set_str("");
        todo_setter.completed.set(false);
        todo_setter.creator.set(Address::ZERO);
        todo_setter.created_at.set(U256::ZERO);

        self.total_todos.set(self.total_todos.get() - U256::from(1));

        log(self.vm(), TodoDeleted {
            id,
            deleter: sender,
        });

        Ok(())
    }

    pub fn get_total_todos(&self) -> U256 {
        self.total_todos.get()
    }

    pub fn get_next_id(&self) -> U256 {
        self.next_id.get()
    }

    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }

    pub fn todo_exists(&self, id: U256) -> bool {
        let todo = self.todos.get(id);
        todo.id.get() != U256::ZERO || id == U256::ZERO
    }

    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), TodoError> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(TodoError::NotOwnerError(NotOwnerError{}));
        }

        self.owner.set(new_owner);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::testing::*;
    use alloc::string::ToString;

    #[no_mangle]
    pub unsafe extern "C" fn emit_log(_pointer: *const u8, _len: usize, _: usize) {}

    #[test]
    fn test_constructor() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        
        contract.constructor();
        
        assert_eq!(contract.get_total_todos(), U256::ZERO);
        assert_eq!(contract.get_next_id(), U256::from(1));
        assert_eq!(contract.get_owner(), vm.msg_sender());
    }

    #[test]
    fn test_create_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let todo_text = "Buy groceries".to_string();
        let result = contract.create_todo(todo_text.clone());
        
        assert!(result.is_ok());
        let todo_id = result.unwrap();
        assert_eq!(todo_id, U256::from(1));
        assert_eq!(contract.get_total_todos(), U256::from(1));
        assert_eq!(contract.get_next_id(), U256::from(2));
        
        // Verify the todo was created correctly
        let (id, text, completed, creator, _created_at) = contract.get_todo(todo_id).unwrap();
        assert_eq!(id, U256::from(1));
        assert_eq!(text, todo_text);
        assert_eq!(completed, false);
        assert_eq!(creator, vm.msg_sender());
    }

    #[test]
    fn test_create_empty_todo_fails() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let result = contract.create_todo("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::EmptyTextError(_)));
    }

    #[test]
    fn test_get_nonexistent_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let result = contract.get_todo(U256::from(999));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::TodoNotFoundError(_)));
    }

    #[test]
    fn test_complete_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        // Create a todo first
        let todo_id = contract.create_todo("Test todo".to_string()).unwrap();
        
        // Complete the todo
        let result = contract.complete_todo(todo_id);
        assert!(result.is_ok());
        
        // Verify it's completed
        let (_, _, completed, _, _) = contract.get_todo(todo_id).unwrap();
        assert_eq!(completed, true);
    }

    #[test]
    fn test_complete_already_completed_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let todo_id = contract.create_todo("Test todo".to_string()).unwrap();
        contract.complete_todo(todo_id).unwrap();
        
        // Try to complete again
        let result = contract.complete_todo(todo_id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::AlreadyCompletedError(_)));
    }

    #[test]
    fn test_complete_nonexistent_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let result = contract.complete_todo(U256::from(999));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::TodoNotFoundError(_)));
    }

    #[test]
    fn test_update_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let todo_id = contract.create_todo("Original text".to_string()).unwrap();
        let new_text = "Updated text".to_string();
        
        let result = contract.update_todo(todo_id, new_text.clone());
        assert!(result.is_ok());
        
        // Verify the update
        let (_, text, _, _, _) = contract.get_todo(todo_id).unwrap();
        assert_eq!(text, new_text);
    }

    #[test]
    fn test_update_todo_with_empty_text() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let todo_id = contract.create_todo("Original text".to_string()).unwrap();
        
        let result = contract.update_todo(todo_id, "".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::EmptyTextError(_)));
    }

    #[test]
    fn test_delete_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let todo_id = contract.create_todo("To be deleted".to_string()).unwrap();
        assert_eq!(contract.get_total_todos(), U256::from(1));
        
        let result = contract.delete_todo(todo_id);
        assert!(result.is_ok());
        assert_eq!(contract.get_total_todos(), U256::ZERO);
        
        // Verify the todo no longer exists
        let result = contract.get_todo(todo_id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::TodoNotFoundError(_)));
    }

    #[test]
    fn test_delete_nonexistent_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let result = contract.delete_todo(U256::from(999));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::TodoNotFoundError(_)));
    }

    #[test]
    fn test_todo_exists() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        // Non-existent todo
        assert_eq!(contract.todo_exists(U256::from(1)), false);
        
        // Create a todo
        let todo_id = contract.create_todo("Test todo".to_string()).unwrap();
        assert_eq!(contract.todo_exists(todo_id), true);
        
        // Delete the todo
        contract.delete_todo(todo_id).unwrap();
        assert_eq!(contract.todo_exists(todo_id), false);
    }

    #[test]
    fn test_unauthorized_operations() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        // Create todo as first user
        let todo_id = contract.create_todo("Test todo".to_string()).unwrap();
        
        // Switch to different user
        let different_user = Address::from([1u8; 20]);
        vm.set_sender(different_user);
        
        // Try to complete todo as different user (should fail)
        let result = contract.complete_todo(todo_id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::UnauthorizedError(_)));
        
        // Try to update todo as different user (should fail)
        let result = contract.update_todo(todo_id, "Updated by wrong user".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::UnauthorizedError(_)));
        
        // Try to delete todo as different user (should fail)
        let result = contract.delete_todo(todo_id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::UnauthorizedError(_)));
    }

    #[test]
    fn test_transfer_ownership() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let original_owner = vm.msg_sender();
        let new_owner = Address::from([1u8; 20]);
        
        // Transfer ownership
        let result = contract.transfer_ownership(new_owner);
        assert!(result.is_ok());
        assert_eq!(contract.get_owner(), new_owner);
        
        // Original owner should no longer be able to transfer ownership
        let result = contract.transfer_ownership(original_owner);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TodoError::NotOwnerError(_)));
    }

    #[test]
    fn test_owner_can_manage_any_todo() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        let owner = vm.msg_sender();
        let user = Address::from([1u8; 20]);
        
        // User creates a todo
        vm.set_sender(user);
        let todo_id = contract.create_todo("User's todo".to_string()).unwrap();
        
        // Owner should be able to complete it
        vm.set_sender(owner);
        let result = contract.complete_todo(todo_id);
        assert!(result.is_ok());
        
        // Owner should be able to update it
        let result = contract.update_todo(todo_id, "Updated by owner".to_string());
        assert!(result.is_ok());
        
        // Owner should be able to delete it
        let result = contract.delete_todo(todo_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_todos() {
        let vm = TestVM::default();
        let mut contract = TodoContract::from(&vm);
        contract.constructor();

        // Create multiple todos
        let todo1_id = contract.create_todo("First todo".to_string()).unwrap();
        let todo2_id = contract.create_todo("Second todo".to_string()).unwrap();
        let todo3_id = contract.create_todo("Third todo".to_string()).unwrap();
        
        assert_eq!(contract.get_total_todos(), U256::from(3));
        assert_eq!(contract.get_next_id(), U256::from(4));
        
        // Verify all todos exist
        assert!(contract.todo_exists(todo1_id));
        assert!(contract.todo_exists(todo2_id));
        assert!(contract.todo_exists(todo3_id));
        
        // Complete one todo
        contract.complete_todo(todo2_id).unwrap();
        let (_, _, completed, _, _) = contract.get_todo(todo2_id).unwrap();
        assert_eq!(completed, true);
        
        // Other todos should still be incomplete
        let (_, _, completed1, _, _) = contract.get_todo(todo1_id).unwrap();
        let (_, _, completed3, _, _) = contract.get_todo(todo3_id).unwrap();
        assert_eq!(completed1, false);
        assert_eq!(completed3, false);
    }
}
