# Pinocchio Template

- Template Pinocchio Program
- Uses `solana-program-test` for the testing framework

Feel free to make PRs to make this template better!

## Notes

Inspirtaion from:
https://github.com/Nagaprasadvr/solana-pinocchio-starter

## Test
```bash
./test.sh
```

## Design Considerations
- No crate should use `pinocchio-template-example-program` directly, instead the sdk should forward all important exports
