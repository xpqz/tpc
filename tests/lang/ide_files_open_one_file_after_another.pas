program ide_files_open_one_file_after_another;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT OTHER.TXT');
end.
