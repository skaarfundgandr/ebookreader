import BookHistory from "../assets/Bookdata/BookHIstory";
import BookSlider from "../assets/Bookdata/BookSlider";
import TheTraitorBook from "../images/thetraitorbook.png";
export default function ContinueRead() {
  const books = [
    { title: "Book 1", author: "Author A", coverImage: TheTraitorBook } ,
    { title: "Book 2", author: "Author B", coverImage: "/covers/book2.jpg" },
    { title: "Book 3", author: "Author C", coverImage: "/covers/book3.jpg" },
    { title: "Book 4", author: "Author D", coverImage: "/covers/book4.jpg" },
    { title: "Book 5", author: "Author E", coverImage: "/covers/book5.jpg" },
    { title: "Book 6", author: "Author F", coverImage: "/covers/book6.jpg" },
    { title: "Book 7", author: "Author G", coverImage: "/covers/book7.jpg" },
    { title: "Book 8", author: "Author H", coverImage: "/covers/book8.jpg" },
    { title: "Book 9", author: "Author I", coverImage: "/covers/book9.jpg" },
    { title: "Book 10", author: "Author J", coverImage: "/covers/book10.jpg" },
  ];

  return (
    <div className="h-full flex flex-col justify-between  items-center w-full">
      <div>
      <BookSlider books={books} visibleCount={5} step={5} />
      </div>
      <div>
      <BookHistory />
      </div>
      <div>
        
      </div>
    </div>
  );
}