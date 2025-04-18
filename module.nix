{ g600 }:
{
  config = {
    security.wrappers = {
      g600 = {
        setuid = true;
        owner = "root";
        group = "root";
        source = "${g600}/bin/g600";
      };
    };
    systemd.user.services = {
      "g600" =
        {
          description = "Service that runs the g600 remapper";
          wantedBy = [ "graphical-session.target" ];
          after = [ "graphical-session-pre.target" ];
          serviceConfig = {
            ExecStart = "/run/wrappers/bin/g600";
          };
        };
    };
  };
}
