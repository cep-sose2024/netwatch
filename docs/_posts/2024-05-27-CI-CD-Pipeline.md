
## What is CI/CD Pipeline

CI/CD automates much or all of the manual human intervention traditionally needed to get new code from a commit into production, encompassing the build, test (including integration tests, unit tests, and regression tests), and deploy phases, as well as infrastructure provisioning. 

## CI/CD Pipeline use

CI/CD is crucial for us, since we use for various testing or even deployment and building scenarios.

### Android Emulator
Using the pipeline we can run an Android Emulator that is used exactly like a normal Android phone, using this feature we can have an idealistic  environment to test our code in. 
This Android Emulator provides neither the SE(Secure Element) and TEE(Trusted Execution Environment), where as they work perfectly according to our requirements.
### Cargo Check

We use the Cargo Check command on our specified directory. It helps us to check the local packages and all of their dependencies for errors, which will essentially compile the packages without performing the final step of code generation, which is faster than running cargo build.

### Rustfmt

Rustfmt is a formatting tool that enforces the standard Rust-community code style. This ensures us that the code is well formatted and readable.

### Jekyll pages

Use use a workflow for building and deploying a Jekyll site to GitHub Pages, it does build this website on commit, of course, only if the commit has something to do with the documentation or /doc directory


## What is meant by continuous testing? [](https://about.gitlab.com/topics/ci-cd/#what-is-meant-by-continuous-testing)

Continuous testing is a software testing practice where tests are continuously run in order to identify bugs as soon as they are introduced into the codebase. In a CI/CD pipeline, continuous testing is typically performed automatically, with each code change triggering a series of tests to ensure that the application is still working as expected. This can help to identify problems early in the development process and prevent them from becoming more difficult and costly to fix later on. Continuous testing can also provide valuable feedback to developers about the quality of their code, helping them to identify and address potential issues before they are released to production.

In continuous testing, various types of tests are performed within the CI/CD pipeline. These can include:

- **Unit testing**, which checks that individual units of code work as expected
- **Integration testing**, which verifies how different modules or services within an application work together
- **Regression testing**, which is performed after a bug is fixed to ensure that specific bug won't occur again
