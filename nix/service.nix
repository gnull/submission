{ config, lib, pkgs, backend, frontend, ... }:

let
  cfg = config.services.submission-web;
in
{
  options.services.submission-web = {
    enable = lib.mkEnableOption "my web application";

    package = lib.mkOption {
      type = lib.types.package;
      default = backend;
      description = "The backend package to use";
    };

    frontend = lib.mkOption {
      type = lib.types.package;
      default = frontend;
      description = "The frontend package to serve";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "Host to bind to";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 8080;
      description = "Port to bind to";
    };

    dataDir = lib.mkOption {
      type = lib.types.path;
      default = "/var/lib/submission-web";
      description = "Directory for database and uploaded files";
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.submission-web = {
      isSystemUser = true;
      group = "submission-web";
      home = cfg.dataDir;
    };

    users.groups.submission-web = {};

    systemd.services.submission-web = {
      description = "Homework submission system";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/submission --static-dir ${cfg.frontend} --uploads ${cfg.dataDir} --database ${cfg.dataDir}/db.sqlite --host ${cfg.host} --port ${toString cfg.port}";
        
        User = "submission-web";
        Group = "submission-web";
        
        # State directory
        StateDirectory = "submission-web";
        # StateDirectoryMode = "0755";
        
        # Hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        DynamicUser = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        # ReadWritePaths = [ cfg.dataDir ];
        
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        
        RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];
        RestrictNamespaces = true;
        LockPersonality = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        RemoveIPC = true;
        
        SystemCallFilter = [ "@system-service" "~@privileged" ];
        
        # Restart policy
        Restart = "on-failure";
        RestartSec = "5s";
      };
    };
  };
}
