# Pristup 🦀⛅🔐

![screenshot of pristup](/img/pristup.png)

A tool that generates temporary AWS Console sign-in URLs. The purpose of this is to enable your users that do not have AWS Console access, temporary access to it without the need for a username and password.

## Getting Started

To get started using this you need to do a few things:

### Get AWS credentials configured locally ☁️

To be able to interact with AWS you need to have a set of AWS Credentials on the machine **Pristup** will run on. The easiest way to get this set up, is by configuring the [AWS CLI](https://aws.amazon.com/cli/). Make sure to install the AWS CLI, and run the `aws configure` [command](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html) to set your credentials.

To verify if you have your AWS credentials set correctly, you can run `aws sts get-caller-identity`:
```bash
darko@devbox [~/]: aws sts get-caller-identity
{
    "UserId": "AIDAXXXXXXXXXXXXXXXXXX5",
    "Account": "123456789999999",
    "Arn": "arn:aws:iam::123456789999999:user/alan-ford"
}
```
Oh, yeah, make sure the user whose credentials you configure has permissions to `AssumeRole` on the specific role you wish to use. This is an important aspect of it, as the role that needs to be assumed, should give this permission to the user you are invoking this application as. 

The easiest way to do this is go to the role you are looking to assume and add such a policy to it's **Trust Relationship**:
```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "",
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::123456789999999:user/alan-ford"
            },
            "Action": "sts:AssumeRole"
        }
    ]
}
```
> ⚠️ Please note: Make sure to replace the **ARN** with the ARN of **your user**.

More information [here](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html).

### Make sure you have Rust installed 🦀

Well that just makes sense, this is a **Rust** application. The easiest way to get started is by using [rustup](https://www.rust-lang.org/tools/install)

### Clone the Repository 💾

As of this date, there is no way to *install* this tool to your machine in a traditional sense. Some elements are hardcoded paths (ie the `pristup.toml` file). And thus, the way you use **Pristup** is by cloning this repository somewhere locally:
```
git clone https://github.com/darko-mesaros/pristup && cd pristup
```

### Running the application 🚀

Finally, to run the application just use the following `cargo` command:
```bash
cargo run <PARAMETERS>
```

## Usage 🔧
```bash
Usage: pristup [OPTIONS]

Options:
  -a, --account <ACCOUNT>
  -r, --role <ROLE>
  -s, --session-name <SESSION_NAME>
  -h, --help                         Print help
  -V, --version                      Print version
```

This will print out the URL to `stdout`. Just click it, copy it, do whatever. 🚀

## Configuration 🛠️

There are two ways of passing the configuration items (account ID, role, and session name) to **Pristup**:

- Using the command line parameters
- Via the `pristup.toml` configuration file

By default, the application looks for command line parameters first. If none are supplied, it gets them from the configuration file located in the root of the repository (`pristup.toml`). This file, with some dummy values is supplied within this repository.

## TODO 📋

- [ ] Handle the configuration file properly (look in $HOME/.config)
- [ ] Use `xdg-open` to automatically open the browser (where possible)
- [ ] Generate temporary CLI credentials for sharing
- [ ] Generate temporary S3 bucket upload permissions

## Version Log 📜

### 0.1.0
- Base functionality
- Generates an URL for temporary console access
- Handles parameters and configuration files (almost)

