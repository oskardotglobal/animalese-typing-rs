{
  lib,
  fetchFromGitHub,
  rustPlatform,
  nix-update-script,
  versionCheckHook,
  xorg,
  alsa-lib-with-plugins,
  pkg-config,
  llvmPackages,
  mold-wrapped,
}:
rustPlatform.buildRustPackage rec {
  pname = "animalese-typing";
  version = "0.1.1";

  src = fetchFromGitHub {
    owner = "oskardotglobal";
    repo = "animalese-typing-rs";
    tag = "v${version}";
    hash = "sha256-qErkyzsWZh5sL8ATD5hnys0Ve6LcxgqFU2ICa0qFNcg=";
  };

  postUnpack = ''
    mkdir -p $out
    cp -r $src/assets/ $out/
  '';

  env.RUSTC_BOOTSTRAP = 1;

  nativeBuildInputs = [
    mold-wrapped
    pkg-config
    llvmPackages.libcxxClang
  ];

  buildInputs = [
    xorg.libX11
    alsa-lib-with-plugins
  ];

  useFetchCargoVendor = true;
  cargoHash = "sha256-KzmHsPxN4IxgX7ALwz0Dl9H46i9ivsbVDdpFHfqpEPk=";

  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgramArg = "--version";
  doInstallCheck = true;

  passthru.updateScript = nix-update-script { };

  meta = {
    description = "Play Animal crossing sounds when you type";
    homepage = "https://github.com/oskardotglobal/animalese-typing-rs";
    changelog = "https://github.com/oskardotglobal/animalese-typing-rs/releases/tag/v${version}";
    mainProgram = "animalese-typing";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [ oskardotglobal ];
  };
}
