### ✅ Step-by-Step Breakdown:

#### **1. Fork the Repo**

Go to the Stylus AssemblyScript example repo (likely [Arbitrum Stylus examples](https://github.com/OffchainLabs/stylus-examples)), then:

* Click on **Fork** (top-right)
* Clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/stylus-examples.git
cd stylus-examples/assemblyscript/
```

---

#### **2. Modify the Codebase**

Open the AssemblyScript file (e.g., `sieve.ts`) and change the logic so instead of computing **all primes up to `n`**, it just **returns the minimum even prime number**, which is always 2.

Update it like this:

```ts
// sieve.ts

// original function might look like this:
export function sieve(n: i32): i32[] {
  const primes = new Array<i32>();
  // ... sieve algorithm
  return primes;
}

// Replace with:
export function minEvenPrime(_n: i32): i32 {
  return 2; // the only even prime
}
```

---

#### **3. Update the `README.md`**

Change the description to reflect the new purpose of the forked example.

Here's an updated `README.md` section:

---

````md
# Stylus AssemblyScript Example – Minimum Even Prime

This fork modifies the original [Stylus AssemblyScript Sieve of Eratosthenes](https://github.com/OffchainLabs/stylus-examples) example to demonstrate a minimal Stylus smart contract that returns the smallest even prime number — which is always **2**.

## 🧠 What Changed

Instead of computing all primes up to a given number `n`, the logic has been simplified to return the only even prime number (2). This showcases a basic AssemblyScript contract in Stylus for educational and testing purposes.

## 📦 Usage

After building the contract:

```bash
npx stylus test
````

Deploy the contract and call:

```ts
contract.minEvenPrime(100); // returns 2
```

## 📁 File Modified

* `assembly/sieve.ts` → replaced `sieve` logic with `minEvenPrime`

````

---

#### **4. Commit & Push**

```bash
git add .
git commit -m "Modified sieve example to return minimum even prime number"
git push origin main
````

## Submission Format : 
TAsk-stylus-js

**GitHub:** [https://github.com/YOUR_USERNAME/stylus-min-even-prime](https://github.com/YOUR_USERNAME/stylus-min-even-prime)

**Description:**
Forked the Stylus AssemblyScript example (Sieve of Eratosthenes) and modified it to return the minimum even prime number (which is always 2), instead of calculating all prime numbers up to a limit. 

This simplified example demonstrates how to write and deploy a minimal smart contract with Stylus using AssemblyScript.
