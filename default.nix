{
  lib,
  pkgs,
  config,
  ...
}:
let
  package = pkgs.callPackage ./package.nix { };
  cfg = config.programs.animalese-typing;
in
{
  meta.maintainers = with lib.maintainers; [ oskardotglobal ];

  options.programs.animalese-typing = {
    enable = lib.mkEnableOption "animalese-typing, play Animal crossing sounds when you type";

    voice = lib.mkOption {
      type = lib.types.int;
      default = 1;
      description = "Voice to use (1-8), where 1-4 are 'female' and 5-8 are 'male'";
    };

    layout = lib.mkOption {
      type = lib.types.enum [
        "ansi-us"
        "iso-de"
      ];
      default = "ansi-us";
      description = "The keyboard layout to use.";
    };
  };

  config = {
    assertions = [
      {
        assertion = cfg.voice >= 1 && cfg.voice <= 8;
        message = "animalese-typing.voice has to be between 1 and 8";
      }
    ];

    systemd.user.services.animalese-typing =
      let
        deps = [
          "session.slice"
          "pipewire.service"
        ];
      in
      lib.mkIf cfg.enable {
        enable = true;
        description = "animalese-typing";

        requires = deps;
        wantedBy = deps;

        serviceConfig = {
          Type = "simple";
          ExecStart = "${lib.getExe package} --voice ${builtins.toString cfg.voice} --layout ${cfg.layout}";
          WorkingDirectory = "${package}";
        };
      };
  };
}
