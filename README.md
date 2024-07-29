# EyeAp

Identity Aware Proxy. This application will allow users of a variety of applications to login, manage their accounts, and also allow the admin to manage users and their access to applications they own|have access to.

This first pass is going to be a learning project, meaning that the above won't be fully implemented, just enough to get an understanding of Fusion auth in a real environment.

## Requirements

### Language

- Rust

### Framework

- Yew.rs

### Data

- Postgres
- Hasura

### Authentication

- Fusion Auth

### Testing

- Playwright

### Other

- Role-based
  - public
  - admin
  - user
- configure fusion auth
  - kickstart

## Stories

- [x] As an unauthenticated admin, I want to log in
  - [x] get user information
  - [x] display user info
- [-] As an admin, I want to register a new app
- [-] As an admin, I want to see a list of apps
- [-] As an unauthenticated user, I want to see a list of applications
- [x] As an unauthenticated user coming from an app, I want to create an account
- [ ] As an authenticated user, I need to confirm that my email is valid when creating an account
- [ ] As an authenticated user, I want to logout
- [ ] As an unauthenticated user coming from an app, I want to login
- [ ] As an authenticated user, I want to purchase a license to use an app
- [ ] As an authenticated user with a license, I want to use an app
- [ ] As an admin, I want to see a list of users
- [ ] As an admin, I want to assign a user to an app
- [ ] As an admin, I want to assign a user to multiple apps
- [ ] As an admin, I want remove a user from an app
- [ ] As an authenticated user, I want to disable my account
- [ ] As an authenticated user, I want to update my account details
- [ ] As an admin, I want to disable a user
- [ ] As an admin, I want to delete a user
- [ ] As an admin, I want to update a user

## Questions for FusionAuth

- [ ] Can I see a list of authenticated browsers (currently logged in on N devices)
  - [ ] Can I choose one to log out of
- [ ] Can you update the default docker image to be compatible with MariaDB?
- [ ] I found some problems in documentation
  - [ ] Kickstart
    - [ ] user.encryptionScheme
      - This isn't encryption it's hashing. It would be nice to have the api name and documentation mention that it's NOT encrypting but rather hashing the passwords.
      - [ ] It would be nice to mention what the default hash type is
- [ ] ui issues
  - [ ] when zoomed in on manager user page, I can only see some of the user tabs, can't select scroll bar to horizontally scroll on firefox
- [ ] Add documentation for completely manual authentication
