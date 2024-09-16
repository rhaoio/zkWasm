## Reproduce constraint system failure.

### Start mongodb

1. `mkdir db`
2. `mongod --dbpath db`

### Run test cli script

```bash
bash test_cli.sh
```

if need to change cuda devices or any other parameters, please modify `test_cli.sh` script.
