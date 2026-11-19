unit Days;
interface
type
  TDay = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
  TDaySet = set of TDay;
const
  Weekend: TDaySet = [Sat, Sun];
function DayName(d: TDay): string;
function NextDay(d: TDay): TDay;
implementation
function DayName(d: TDay): string;
begin
  case d of
    Mon: DayName := 'Monday';
    Tue: DayName := 'Tuesday';
    Wed: DayName := 'Wednesday';
    Thu: DayName := 'Thursday';
    Fri: DayName := 'Friday';
    Sat: DayName := 'Saturday';
    Sun: DayName := 'Sunday';
  end;
end;
function NextDay(d: TDay): TDay;
begin
  if d = Sun then NextDay := Mon else NextDay := Succ(d);
end;
end.
