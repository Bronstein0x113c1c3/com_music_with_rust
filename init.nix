{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
    # something = ["something" "abc"];

  buildInputs = [
    pkgs.pkg-config
    pkgs.portaudio
    pkgs.git
    pkgs.alsa-lib
    pkgs.rustup
  ];

  # Add Go to the PATH when the shell starts
  shellHook = ''
  echo $something   
  '';
}