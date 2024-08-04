# Orca Cave Exposee

## the Goal!
Easy configigurable Orca Profiles from warios Hackspaces (sources).

## What its doing?
### Providing
- Path to orca configs (optional)
  - can be derived by system identifyer
- Path to config.json file


### Doing
- reading a config.json file
  - a list of Space(source)-Names as key and Profile-Prefix!!!
  - each holding:
    - Git(hub) adress
    - deployment !publik!key
  - Each profile file **neads to** start with the Profile-Prefix == <Space-Name>
- checking if orcapath is legit:
  - all folders are in there:
  - machine, filament, profile
- checking if all Profile-Prefixes are diffrent
- checking if all remote Profiles impl. the Profil-Prefix naming convention
- 

