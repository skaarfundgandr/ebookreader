import React from "react";
import ContinueRead from "../components/ContinueRead";
import LastRead from "../components/LastRead";
import QuoteGenerator from "../components/QuoteGenerator";

export default function HomePage() {
  return (
    <div className="h-full grid grid-cols-[1.5fr_1fr] grid-rows-[350px_1fr] [grid-template-areas:'quote last''continue continue'] flex-1">
       <QuoteGenerator />
       <LastRead/>
       <ContinueRead />
    </div>
  );
}