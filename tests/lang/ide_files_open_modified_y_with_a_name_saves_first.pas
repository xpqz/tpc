program ide_files_open_modified_y_with_a_name_saves_first;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT OTHER.TXT');
end.
