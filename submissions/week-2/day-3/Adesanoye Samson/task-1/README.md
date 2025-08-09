# Task:Usage of Enum, Result & the Question Mark Operator

## Requirements

- Determine if an employee can access web3bridge garage using a digital keycard.
- Employees that **can access** the building are:
  - Media team
  - IT department employees
  - Managers
- Other employees that **work at the company** are:
  - Social media team
  - Technician supervisors
  - Kitchen staff
- Ensure that **terminated employees cannot access** the building regardless of their position.

## Notes

- Use an `enum` to represent all types of employees.
- Use a `struct` to store:
  - the employee type
  - whether they are still employed
- Use a function that returns a `Result` to determine if the employee may enter the building.
- Print whether the employee may access the building:
  - Must use a function that utilizes the **question mark (`?`) operator** to do this.

---