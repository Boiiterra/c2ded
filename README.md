# c2ded

C2 death detector \
Detects sus traffic and bans it notifying admins.

## TODO

As all traffic travels through us we only need to monitor and check if for suspicious activity.

- [ ] main application
  - [ ] \(choice 1.1) get private keys [^1]
  - [ ] \(choice 1.2) use our public key for stuff coming to us [^1]
  - [ ] receiver
    - [ ] get something
    - [ ] decrypt or copy
    - [ ] analyze
    - [ ] verdict
    - [ ] encrypt?
    - [ ] pass to sender
  - [ ] sender
    - [ ] send encrypted to the destination

- [ ] admin stuff [^2]
  - [ ] show verdicts
  - [ ] see banned IPs
  - [ ] remove ban from IPs

- [ ] give this little project name

- [ ] \(extra) Finish toml file.

[^1]: Need to decide between this two which one to use.
[^2]: This one may be part of main application or something separate.
