# project_name

project_badge

> project_keynote

---

## Features

*   **feature1_name**: feature1_detail

## Stack

*   **Frontend:**   project_frontend
*   **Backend:**    project_backend
*   **Database:**   project_database
*   **Styling:**    project_styling

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

Get the latest release from github_url

>**Note:** The name of the executable will be "project_name-*", where * is the version number, and build version,
> remember to type the full name when executing like ```./project_name-* -V``` , or rename it from "project_name-*" to "project_name"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./project_name```


Or download from command line, like this:


#### Download

```shell
curl -L github_releases_url -o project_name
chmod +x project_name
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./project_name -V
```

## Building from Source

1. **Pre-requisites:**  
   Need to install Rust


2. **Clone the repository:**

```shell
git clone github_url.git
cd project_name
```


3. **Build**:

   Probably just run:

```shell
cargo build --release
```


4. **Done!:**  
   Now test the binary with:

```shell
./project_name -V
```

5. Add to alias:    
   If you want to add it so that you can run it directly without the './', then do this:

If using Bash:

```shell
mv project_name .local/bin/
echo 'alias project_name="./.local/bin/project_name"' >> .bashrc
```

If using Zsh:

```shell
mv project_name ~/local/bin/project_name
echo 'alias project_name="./.local/bin/project_name"' >> .bashrc
```

If using Fish:

```shell
mv project_name ~/local/bin/project_name
abbr -a project_name "./.local/bin/project_name"
```

## Contributors
*   contributer1_badge

> Coded and tested in operating_systems_working, should work in any distro.

