This is a Mac OSX specific issue: if the procstat executable after placement in /usr/local/bin cannot be executed because OSX will complain 'Apple cannot check app for malicious software', perform the following steps:

1. If the current OSX user does not have sysadmin rights, add these or choose a user that has sysadmin rights.
2. Choose the option to open the location of the executable in Finder from the malicious software dialogue that was started.
3. Control-click the procstat application and choose 'open' from the menu.
4. Click 'open'.

The application is now added as a security exception to the security settings.

(https://support.apple.com/en-gb/guide/mac-help/mchleab3a043/mac)
