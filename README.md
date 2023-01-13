# fvwm menu maker

---

### Philosophy

Remember to do one thing and do it well.
> Better to have 100 functions operate on one data structure than to have 10 functions operate on 10 data structures.
  
- Make each function do **one** thing well
- Expect the output of very function to become the input of another (yet unknown) function
- Design functions to be tested early

---

# Steps, Path, Roadmap

- Read applications directory
- Get all the files that have a .desktop extension
- Read each file
- Parse each file for specific properties
- Create a menu item based on the applications category
- Write to menu file

---

## Focus

### Application Directory
- get location of applications directory
  - Is there a .env file in $HOME dir? 
    - If yes, **read** .env **file** and use the app dir path there
    - If no .env the check argv for location : string or path
- **Verify path** exists; if not, exit
- List application directory
- get all files with .desktop extension

### Read desktop files
- Cycle through file list and read each file
- Parse out all required (needed) data
  - Type
  - Category
  - Name
  - Exec
  - Terminal (bool)

### Create Menu
- generate structs
- organize by category
- create menu file

---

## Percolate 

- Read and parse each file concurrently or in parallel
- Insert menu into .fvwmrc file