# lankat

[project_badge]([![project_name](https://img.shields.io/badge/GitHub-project_name-green?style=plastic)](https://www.github.com/Tylerr-D/project_name))

> send messages! (through lan)

---

## Features

*   **Browser**: You can text anyone via a browser
*   **TCP**: You can text anyone using lankat in your lan via tcp

## Stack

*   **Frontend:**   RataTUI
*   **Backend:**    Rust
*   **Styling:**    Compartmentalizing? Sectioning? stuff idk

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

Get the latest release from [https://github.com/Tylerr-D/lankat](https://github.com/Tylerr-D/lankat)

>**Note:** The name of the executable will be "lankat-*", where * is the version number, and build version,
> remember to type the full name when executing like ```./lankat-* -V``` , or rename it from "lankat-*" to "lankat"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./lankat```


Or download from command line, like this:


#### Download

```shell
curl -L github_releases_url -o lankat
chmod +x lankat
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./lankat -V
```

## Building from Source

1. **Pre-requisites:**  
   Need to install Rust


2. **Clone the repository:**

```shell
git clone https://github.com/Tylerr-D/lankat.git
cd lankat
```


3. **Build**:

   Probably just run:

```shell
cargo build --release
```


4. **Done!:**  
   Now test the binary with:

```shell
./lankat -V
```

5. Add to alias:    
   If you want to add it so that you can run it directly without the './', then do this:

If using Bash:

```shell
mv lankat .local/bin/
echo 'alias lankat="./.local/bin/lankat"' >> .bashrc
```

If using Zsh:

```shell
mv lankat ~/local/bin/lankat
echo 'alias lankat="./.local/bin/lankat"' >> .bashrc
```

If using Fish:

```shell
mv lankat ~/local/bin/lankat
abbr -a lankat "./.local/bin/lankat"
```

## Contributors
*   **[![Amaan](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **[![Ruster](https://img.shields.io/badge/GitHub-Ruster-orange?style=plastic)](https://www.github.com/Tylerr-D)**

> Coded and tested in operating_systems_working, should work in any distro.

