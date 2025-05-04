{occasion}: {
  pkgs,
  config,
  lib ? pkgs.lib,
  ...
}: let
  inherit (lib) mkEnableOption mkOption types mkIf literalExpression;
  cfg = config.programs.occasion;
  json = pkgs.formats.json {};
in {
  options.programs.occasion = {
    enable = mkEnableOption "Enable occasion.";
    package = mkOption {
      description = "Package for occasion.";
      type = types.package;
      default = occasion;
    };
    settings = mkOption {
      description = "JSON config for occasion (occasions.json)";
      type = json.type;
      default = {};
      example = literalExpression ''
        {
          dates = [
            {
              message = "hello friday!";
              time = {
                day_of = {
                  week = ["Friday"];
                };
              };
            }
          ];
          multiple_behavior = {
            all = {
              seperator = "";
            };
          };
        }
      '';
    };
  };
  config = mkIf cfg.enable {
    home.packages = [cfg.package];
    xdg.configFile."occasions.json" = mkIf (cfg.settings != {}) {
      source = json.generate "occasions.json" cfg.settings;
    };
    home.activation = mkIf (cfg.settings != {}) {
      checkOccasionConfigFile = lib.hm.dag.entryBefore ["writeBoundary"] ''
        if ! output=$(${cfg.package}/bin/occasion --check 2>&1); then
          echo "❌ Occasion config check failed:"
          echo "$output"
          exit 1
        else
          echo "✅ Occasion config check passed."
        fi
      '';
    };
  };
}
