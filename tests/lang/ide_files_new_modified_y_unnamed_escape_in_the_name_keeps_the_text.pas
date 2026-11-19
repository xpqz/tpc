program ide_files_new_modified_y_unnamed_escape_in_the_name_keeps_the_text;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('NONAME.PAS');
end.
