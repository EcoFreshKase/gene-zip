; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "gene-zip"
#define MyAppVersion "1.0.0"
#define MyAppExeName "gene-zip.exe"
#define MyAppAssocName MyAppName
#define MyAppAssocExt ".fasta"
#define MyAppAssocKey MyAppAssocName
#define ExecutablePath "C:\Users\alexa\Desktop\Coding Projekte\g-zip\gene-zip\target\release\gene-zip.exe" ; Add Path to the executable (absolute or relative to this file)

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{CC322B1C-B294-49C8-B9A7-78A7CCDCF51F}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
ChangesAssociations=yes
DisableProgramGroupPage=yes
InfoBeforeFile=infobefore.txt
InfoAfterFile=infoafter.txt
PrivilegesRequired=admin
OutputBaseFilename=gene-zip installer
; SetupIconFile=icon.ico ; Remove this comment to add an Icon to the Installer
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: {#ExecutablePath} ; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Registry]  
Root: HKCR; Subkey: "*\shell\{#MyAppAssocKey}"; ValueType: string; ValueData: "start {#MyAppName}"; Flags: uninsdeletekey
Root: HKCR; Subkey: "*\shell\{#MyAppAssocKey}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\{#MyAppExeName}"
Root: HKCR; Subkey: "*\shell\{#MyAppAssocKey}\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%1"""

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"

