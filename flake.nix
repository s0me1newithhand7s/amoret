{
  description = "hand7s'es flake";

  nixConfig = {
    max-jobs = "auto";
    builders = "";
    require-sigs = true;
    sandbox = true;
    sandbox-fallback = false;
    auto-optimise-store = true;

    allowed-users = [
      "@wheel"
    ];

    trusted-users = [
      "root"
      "@wheel"
    ];

    experimental-features = [
      "nix-command"
      "flakes"
    ];

    extra-substituters = [
      "https://cache.nixos.org"
      "https://nix-community.cachix.org"
      "https://devenv.cachix.org"
    ];

    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    "cachix" = {
      flake = true;
      type = "github";
      owner = "cachix";
      repo = "cachix";

      inputs = {
        "nixpkgs" = {
          follows = "nixpkgs";
        };
      };
    };

    "devenv" = {
      flake = true;
      type = "github";
      owner = "cachix";
      repo = "devenv";

      inputs = {
        "nixpkgs" = {
          follows = "nixpkgs";
        };
      };
    };

    "devenv-root" = {
      flake = false;
      url = "file+file:///dev/null";
    };

    "flake-parts" = {
      flake = true;
      type = "github";
      owner = "hercules-ci";
      repo = "flake-parts";
    };

    "git-hooks-nix" = {
      flake = true;
      type = "github";
      owner = "cachix";
      repo = "git-hooks.nix";

      inputs = {
        "nixpkgs" = {
          follows = "nixpkgs";
        };
      };
    };

    "nixpkgs" = {
      flake = true;
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };

    "treefmt-nix" = {
      flake = true;
      type = "github";
      owner = "numtide";
      repo = "treefmt-nix";

      inputs = {
        "nixpkgs" = {
          follows = "nixpkgs";
        };
      };
    };
  };

  outputs = inputs @ {self, ...}:
    inputs.flake-parts.lib.mkFlake {
      inherit
        inputs
        self
        ;
    } {
      debug = true;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "riscv64-linux"
      ];

      imports = [
        inputs."treefmt-nix".flakeModule
        inputs."devenv".flakeModule
      ];

      perSystem = {
        config,
        pkgs,
        lib,
        ...
      }: {
        # amoret - discord rpc
        packages = {
          "default" = pkgs.rustPlatform.buildRustPackage {
            pname = "amoret";
            version = "0.1.1";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

          "amoret" = config.packages."default";

          "testRun" = pkgs.writeShellApplication {
            name = "testRun";

            runtimeInputs = with pkgs; [
              git
              prek
              cargo
            ];

            text = ''
              prek run --all-files --fail-fast ;
              cargo check ;
            '';
          };
        };

        # numtide/treefmt-nix, treefmt integrated into nix
        treefmt = {
          flakeFormatter = true;

          programs = {
            "alejandra" = {
              enable = true;
              priority = 1;
              includes = [
                "*.nix"
              ];
            };

            "statix" = {
              enable = true;
              priority = 1;
              includes = [
                "*.nix"
              ];
            };

            "deadnix" = {
              enable = true;
              priority = 1;
              includes = [
                "*.nix"
              ];
            };

            "mdformat" = {
              enable = true;
              priority = 2;
              includes = [
                "*.md"
              ];
            };

            "rustfmt" = {
              enable = true;
              priority = 2;
              includes = [
                ".rs"
              ];
            };
          };

          settings = {
            global = {
              on-unmatched = "warn";
              excludes = [
                ".gitignore"
              ];
            };
          };
        };

        # cachix/git-hooks-nix, pre-commit-hooks integrated into nix
        pre-commit = {
          check = {
            enable = true;
          };

          settings = {
            enable = true;
            package = pkgs.prek;
            gitPackage = pkgs.git;

            hooks = {
              "alejandra" = {
                enable = true;
                settings = {
                  verbosity = "quiet";
                  check = true;
                };
              };

              "deadnix" = {
                enable = true;
                settings = {
                  edit = false;
                };
              };

              "statix" = {
                enable = true;
              };

              "markdownlint" = {
                enable = true;
                package = pkgs.markdownlint-cli;
              };

              "gitlint" = {
                enable = true;
              };

              "rustfmt" = {
                enable = true;
              };

              "clippy" = {
                enable = true;
              };
            };
          };
        };

        # cachix/devenv, basically a devShells, even better than numtide/devshells
        devenv = {
          shells = {
            "default" = {
              enterShell = config.pre-commit.shellHook;

              enterTest = ''
                ${lib.getExe config.packages."testRun"}
              '';

              languages = {
                rust = {
                  enable = true;
                  channel = "nixpkgs";
                };
              };

              cachix = {
                pull = [
                  "nix-community"
                  "devenv"
                ];
              };

              packages =
                [
                  pkgs.cachix
                  config.treefmt.build.wrapper
                ]
                ++ config.pre-commit.settings.enabledPackages;
            };
          };
        };
      };
    };
}
