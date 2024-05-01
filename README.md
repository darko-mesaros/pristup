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

Now, you need some additional packages to be able to compile **pristup**. Namely you need the `build-essential` (or similar) package group. Depending on your operating system, and package manager the name may differ.

**Ubuntu/Debian:**
```
sudo apt install build-essential
```

**Arch Linux:**
```
sudo pacman -S base-devel
```

**MacOS:**
```
xcode-select --install
```

**Amazon Linux/Red Hat/CentOS:**
```
yum groupinstall "Development Tools"
```

**Additionally**, you *may* need the `pkg-config` and `libssl-dev` packages (or their equivalents), depending on your operating system.

### Clone the Repository 💾

To install this package, you can just run `cargo install pristup`

This will install the compiled binary into your `$CARGO_HOME/bin` directory. If you have the `$PATH` set up correctly you should be able to run it now. But before you do ...

Let's initialize the configuration. Because **pristup** uses a configuration file (`pristup.toml`) it needs to be stored inside of your `$HOME/.config/bedrust` directory. *Now*, you can do this manually, but we have a feature to do it for you. Just run:
```
pristup --init
```
After entering the AWS Account ID and the Role you wish to assume, it will create all the necessary files for you to be able to use **pristup**. There is no need to modify these files, unless you want to.

### Running the application 🚀

Finally, to run the application just run:
```bash
pristup
```

## Usage 🔧
```bash
Usage: pristup [OPTIONS]

Options:
  -a, --account <ACCOUNT>
  -r, --role <ROLE>
  -s, --session-name <SESSION_NAME>
  -t  --timeout <TIMEOUT>
      --init
  -h, --help                         Print help
  -V, --version                      Print version
```

This will print out the URL to `stdout`. Just click it, copy it, do whatever. 🚀

## Configuration 🛠️

There are two ways of passing the configuration items (account ID, role, and session name) to **Pristup**:

- Using the command line parameters
- Via the `pristup.toml` configuration file that is located in your `$HOME/.config/pristup` directory

By default, the application looks for command line parameters first. If none are supplied, it gets them from the configuration file. 
