unit Wrap;

interface

function Shout(S: String): String;
function Padded(N: LongInt): String;

implementation

uses Utils;

function Shout(S: String): String;
begin
  Shout := UpperCase(Trim(S)) + '!';
end;

function Padded(N: LongInt): String;
begin
  Padded := LeadingZero(N, 4);
end;

end.
