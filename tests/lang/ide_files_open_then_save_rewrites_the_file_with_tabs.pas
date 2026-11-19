program ide_files_open_then_save_rewrites_the_file_with_tabs;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('TABS.TXT');
end.
