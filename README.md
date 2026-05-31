# Lunch Of the Day

### Installation
In order to install this project, you first need to make sure that you have `rust` installed on your system. If you don't already, run the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You will also need to aquire an API key, which can be done on this [website](https://lunchaimjardevi.com/api/index.php). That key needs to be pasted into `.env`.
See the `.env.exaple` file for the required format.


The last step in installing `lod` is to run the following command from the root of this project:
```bash
cargo install --path .
```

All should now be done! To use this programme, simply run:
```bash
lod
```
