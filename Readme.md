# Rust/AWS lambda

##  Introduction
Hands on Rust/AWS lambda.

https://dev.classmethod.jp/articles/rust-lambda-cdk/

## Instruction
```
# build rust code.
$ npm run build

# set up aws region 
$ export AWS_REGION=<your target region>

# run built rust code
$ npm run cdk:bootstrap

# deploy to your aws environment
$ npm run deploy

$ aws lambda invoke \
--function-name rust-lambda-cdk-main \
--cli-binary-format raw-in-base64-out \
--region $AWS_REGION \
--payload '{}' \
tmp-output.json > /dev/null && cat tmp-output.json && rm tmp-output.json

```
### LocalStack
```
# execute docker-compose up -d
$ npm run cdklocal:start
```