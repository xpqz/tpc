program ConsolePromptThenReadAnswers;
var
  name: string;
  age: Integer;
begin
  Write('Name? ');
  ReadLn(name);
  Write('Age? ');
  ReadLn(age);
  WriteLn('Hello, ', name, '. Next year you are ', age + 1, '.');
end.
