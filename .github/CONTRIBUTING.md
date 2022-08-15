# Contributing Guide

## Table of Contents

* [Code of Conduct](#code-of-conduct)
* [Contributing Code](#contributing-code)
	+ [Style Guide](#style-guide)
	+ [Getting Started](#getting-started)
	+ [Finding an Issue](#finding-an-issue)
* [Important Resources](#important-resources)
* [Questions](#questions)
* [Feature Requests](#feature-requests)
* [Reporting Bugs](#reporting-bugs)
* [Improving Documentation](#reporting-bugs)
* [Pull Request Process](#pull-request-process)
	+ [Addressing Feedback](#addressing-feedback)
* [Community](#community)

## Code of Conduct

[Code of Conduct][Code of Conduct]

## Contributing Code

### Style Guide

It is important that you read the [Style Guide][Style Guide] before you contribute any code. This ensures consistency throughout the codebase and avoids (most) debates in regards to formatting preferences. Thank god for code formatters.

### Getting Started

You will need to fork the main repository to work on your changes. Simply navigate to the GitHub page and click the "Fork" button at the top. Once you've forked the repository, you can clone your new repository and start making edits.

In git it is best to isolate each topic or feature into a “topic branch”. While individual commits allow you control over how small individual changes are made to the code, branches are a great way to group a set of commits all related to one feature together, or to isolate different efforts when you might be working on multiple topics at the same time.

While it takes some experience to get the right feel about how to break up commits, a topic branch should be limited in scope to a single issue

```bash
# Checkout the master branch - you want your new branch to come from master
git checkout master

# Create a new branch named newfeature (give your branch its own simple informative name)
git branch newfeature

# Switch to your new branch
git checkout newfeature
```

For more information on the GitHub fork and pull-request processes, [please see this helpful guide][Pull Request Guide].

### Finding an Issue

The list of outstanding feature requests and bugs can be found on our on our [GitHub issue tracker][Issues]. Pick an issue that you think you can accomplish and add a comment that you are attempting to do it.

Bug Reports can be submitted in the `Issues` tab.

## Important Resources

[`Cargo.toml`][Cargo.toml] provides a list of crates this program depends on.
 
## Questions

Please submit questions in the `issues` tab and apply the `question` label.

## Feature Requests

Please submit feature requests in the `issues` tab by filling out the Feature Request template.

Please provide the feature you would like to see, why you need it, and how it will work. Discuss your ideas transparently so I can better understand why this feature is necessary.

Major changes that you wish to contribute to the project should be discussed first in an GitHub issue that clearly outlines the changes and benefits of the feature.

Small changes can directly be crafted and submitted to the GitHub Repository as a pull request. See the section about pull request Submission Guidelines, and for detailed information the core development documentation.

## Reporting Bugs

Please report bugs requests in the `issues` tab by filling out the Bug Report template.

Before you submit your issue, please [search the issue archive][Issues] - maybe your question or issue has already been identified or addressed.

**Be sure to include a screenshot or a code block of the *entire* traceback of the error in the Bug Report template.**

## Improving Documentation

Should you have a suggestion for the documentation, you can open an issue with an `enhancement` label and outline the problem or improvement you have - however, creating the doc fix yourself is much better!

For large fixes, please build and test the documentation before submitting the pull request to be sure you haven't accidentally introduced any layout or formatting issues.

For new features, please include screenshots or a demo GIF of the feature running in a terminal.

## Pull Request Process

When you are ready to generate a pull request, either for preliminary review, or for consideration of merging into the project you must first push your local topic branch back up to GitHub:

```bash
git push origin newfeature
```

Once you've committed and pushed all of your changes to GitHub, go to the page for your fork on GitHub, select your development branch, and click the pull request button. If you need to make any adjustments to your pull request, just push the updates to your branch. Your pull request will automatically track the changes on your development branch and update.

1. Completely fill out the `PULL_REQUEST_TEMPLATE.md`. Make sure you go through the checklist to ensure you have followed the necessary procedures.

2. Update the `README.md` with details of changes and include a walkthrough accommodated by screenshots.
   
3. Increase the version numbers in `requirements.txt`, if applicable, and the README.md to the new version that this pull request would represent.

### Addressing Feedback

Once a pull request has been submitted, your changes will be reviewed and constructive feedback may be provided. Feedback is not meant as an attack, but to help make sure the highest-quality code makes it into my project. Changes will be approved once required feedback has been addressed.

If you are asked to "rebase" your pull request, this means a lot of code has changed, and that you need to update your fork so it's easier to merge.

To update your forked repository, follow these steps:

```bash
# Fetch upstream master and merge with your repo's master branch
git fetch upstream
git checkout master
git merge upstream/master

# If there were any new commits, rebase your development branch
git checkout newfeature
git rebase master
```

If too much code has changed for git to automatically apply your branches changes to the new master, you will need to manually resolve the merge conflicts yourself.

Once your new branch has no conflicts and works correctly, you can override your old branch using this command:

```bash
git push -f
```

Note that this will overwrite the old branch on the server, so make sure you are happy with your changes first!

## Community

- If you know the answer (or a better solution to an existing answer) to a question that was listed in the `Issues` tab, feel free to contribute.

- If you have a better implementation of existing code that will improve runtime or introduces better logic, follow the pull request process to suggest improvements.

- If you liked this program, please spread the word and share it with others! 

[Cargo.toml]: https://github.com/JosephLai241/himitsu/blob/main/Cargo.toml
[Code of Conduct]: CODE_OF_CONDUCT.md
[How To Contribute]: https://egghead.io/series/how-to-contribute-to-an-open-source-project-on-github
[Issues]: https://github.com/JosephLai241/himitsu/issues
[Pull Request Guide]: https://gist.github.com/Chaser324/ce0505fbed06b947d962
[Style Guide]: STYLE_GUIDE.md
