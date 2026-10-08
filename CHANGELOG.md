# Changelog

## [27.1.1]

# Changes
- Moved the API documentation from the README to Rust Docs comments

# Fixes

## [27.1.0]

# Changes
- Refactored coordinate helper functions for better performance
- Adds tests for the parser
- Adds tests for examples to the CI
   - Because the tests are running games against the Java random player they are effectifly also tests for the client.
- Cleans up the CI and ommits duplicate runs

# Breaking API changes
- The coordinate helper functions now mutate the given vector. See for example [here](https://github.com/software-challenge/player_rust/commit/cffcc9abeb560dd41ee372d6f34acb4c07aee02d#diff-69c63c0aa0c9b3c376b7e9df60c2982dfb31c1beb01040042cca4ca80124c76f). 

# Fixes
- [#40](https://github.com/software-challenge/player_rust/issues/40)
- Edge case communication error

## [27.0.6]

# Changes
- Adds more CI tests
- Adds deployment action
- Corrected the minimum rust version 

# Fixes
- #28

## [27.0.5]

# Changes
- Clippy warnings fixed
- Clippy checks added to the CI for better quality code

# Fixes
- #25


## [27.0.4]

# Changes
- Replace xml-rs with quick-xml for better performance

# Fixes 


## [27.0.3]

# Changes
- Tests
- Performance update for calculating points
- Fix readme

# Fixes 

## [27.0.2]

# Changes
- Functions to calculate points by team and color in game state
- Function to calculate tiles of specified color
- Refactoring

# Fixes
- #12 

## [27.0.1]

# Changes
- Added some potentially missing derives
- Refactored code that switches unnecessarily between isize and usize

# Fixes
- #5 
