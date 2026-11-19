program ide_errors_unknown_code_has_no_text;
uses Crt, Errors;

procedure Show(Code: Integer);
begin
  WriteLn(Code, ' [', ErrorText(Code), '] ', Length(ErrorText(Code)));
end;

begin
  Show(0);
  Show(1);
  Show(11);
  Show(201);
  Show(32767);
  Show(-1);
end.
