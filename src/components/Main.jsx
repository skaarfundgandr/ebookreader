import AddItem from "./AddItem";
import Community from "./Community";
import ContinueRead from "./ContinueRead";
import LastRead from "./LastRead";
import QuoteGenerator from "./QuoteGenerator";

export default function Main() {
  return (
    <div className="h-full grid grid-cols-[1.5fr_1fr] grid-rows-[350px_1fr] [grid-template-areas:'quote last''continue continue'] flex-1">
       <QuoteGenerator />
       <LastRead/>
       <ContinueRead />
    </div>
  );
}