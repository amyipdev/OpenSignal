# OpenSignal

This is a pre-alpha version of OpenSignal, provisionally licensed under GPLv2+. I hope to have a better version of this out some time in late May.

You need to have installed and set up [Signal Desktop](https://signal.org/download/), [signal-cli](https://github.com/AsamK/signal-cli), and either [Nix](https://nixos.org/download) or [Lix](https://lix.systems/install). You also need to [enable nix-command and flakes](https://nixos.wiki/wiki/Flakes).

This application has only been tested on NixOS, but should work without issue on any Linux-based system with Nix/Lix installed. It may work on macOS, but I have not tested this. 

It will not work on regular Windows (a port may be made in the future), but you can install Nix in [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install); if you manage to get Signal Desktop and signal-cli working in WSL2 as well, you can use OpenSignal in WSL2, but this may be incredibly difficult. Instead, I recommend [making a Linux virtual machine](https://itsfoss.com/install-fedora-in-virtualbox/) and installing everything there.

Once you've got the dependencies set up, just run:

```sh
nix run github:amyipdev/OpenSignal
```

The GUI will give you instructions from there.
