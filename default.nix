let
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/2a1c9124f52e18e7829bea62e8cc476572a0bd32.tar.gz";
    sha256 = "sha256:07q0ykvg3wkg35ax7vap7zfsvpfmi5y48c0dvnkww78690a9gwgj";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";

    holochainVersion = {
      rev = "06c82ade3128ccc73b7b24a137d8fad3756927ef";
      sha256 = "sha256:1fykfqslr7lhbp11wbl7cz5pmygw9wmhlkvvnfn9ig9ddr7nq6sw";
      cargoSha256 = "sha256:11s50qq7719grgijnw2z2wi27xa918ycjnsmcd5a8c2kvf4al3yw";
      bins = {
        holochain = "holochain";
        hc = "hc";
        kitsune-p2p-proxy = "kitsune_p2p/proxy";
      };

      lairKeystoreHashes = {
        sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
        cargoSha256 = "sha256:1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
      };
    };
    holochainOtherDepsNames = ["lair-keystore"];
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
  ];
}