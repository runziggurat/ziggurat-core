{ pkgs, ... }:
with pkgs.lib;
let
  ciScriptTemplate = { name, command }: ''
    echo -e "${name}:"
    echo "Running \"${command}\"..."
    ${command} \
    && echo -e "\033[0;32mSucceeded!\033[0m" \
    || echo -e "\033[0;31mFailed!\033[0m"
  '';

  # Wrapper function around `mkCiScriptsBin` and `mkCiScriptBin`.
  # Outputs a list containing the results of both functions.
  mkCiScripts = s: (mkCiScriptsBin s) ++ [ (mkCiAllScriptBin s) ];

  # Writes an executable script for each entry in an attribute set.
  mkCiScriptsBin = s: attrsets.mapAttrsToList
    (name: command: (pkgs.writeScriptBin "ci-${name}"
      (ciScriptTemplate { inherit name command; })))
    s;

  # Writes an executable script that will run all entries in an attribute set.
  mkCiAllScriptBin = s: pkgs.writeScriptBin "ci-all"
    (strings.concatStringsSep "\necho\n"
      (attrsets.mapAttrsToList
        (name: command: (ciScriptTemplate { inherit name command; }))
        s));
in
{
  inherit mkCiScripts;
}
