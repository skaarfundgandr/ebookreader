import BookHeader from "../components/BookDetails/BookHeader";

export default function BookPage() {

    const handleMenu = () => {
        AlertDialog("Menu option");
    }

    const handleBack = () => {
        AlertDialog("Back option");
    }

    return( 
        <div className="bg-black">
            <BookHeader onMenu={handleMenu} onBack={handleBack}/>
        </div>
    )
}