program ide_files_new_then_type_and_save_uses_a_new_name;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('GOOD.TXT NEW1.PAS');
end.
