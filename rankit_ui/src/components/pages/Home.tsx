import "./Home.css";

export function Home() {
  return (
    <div className="Home" >

      <h2>About</h2>
      <p className="blog">
        Rankit is a ranking site, similar to a review site, but with a few twists.
        Rankit does not merely rank games, TV shows, or movies.
        Rather, Rankit can rank anything in any category that it belongs to.
        Also, rather that using a traditional rating system like letter grades or stars, Rankit derives
        the quality things through comparisons. A user starts a poll and is presented with a list
        of random things that all belong to the same category. They then filter out the things they're
        unfamiliar with. Finally, they sort them in order of best to worst.
      </p>

      <h2>Things</h2>
      <p className="blog">
        Things can be, well, anything! Spiders, apples, socks, fingernails, you name it! Things typically
        belong to one or more categories, and have independent rankings in each. 
      </p>

      <h2>Categories</h2>
      <p className="blog">
        Categories describe things.
        Say you have two categories: <b>Scary</b>, <b>Cute</b>.<br/>
        <b>Scary</b> consists of the following things: Spiders, ghosts, clowns.<br/>
        <b>Cute</b> consists of the following things: Puppies, kittens, spiders (well, to some brave souls).<br/>
        Spiders belongs to both categories, so it gets a ranking in each.
        You'd likely see spiders rank high in the scary category, but low in the cute category.
      </p>
    </div>
  )
}
