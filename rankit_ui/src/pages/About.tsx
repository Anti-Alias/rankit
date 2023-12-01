import styles from './About.module.css';

const About = () => (
    <>
        <h1>About</h1>
        <p className={styles.paragraph}>
            Thingelo is a site dedicated to rating anything and everything in the universe.
            Movies, games, paintings, songs, food, etc.
            Rather than using a traditional rating scale, Thingelo uses the ELO rating system to estimate the quality of its content.
            When using the site, a user is prompted with two <b>Things</b> belonging to the same <b>Category</b>.
            The user then selects their preference, and the chosen <b>Thing's</b> ELO score goes up while the other's goes down.
            (Note: A <b>Thing's</b> ELO score in one <b>Category</b> may be completely different in another.)
        </p>

        <h1>Things</h1>
        <p className={styles.paragraph}>
            Rateable contents are called <b>Things</b>.
            Each is user-generated and comes with an associated image representing it.
            A red-delicious apple is a <b>Thing</b>.
            The movie <i>The Godfather</i> is a <b>Thing</b>.
            The earth itself is a <b>Thing</b>.
            A <b>Thing</b> can belong to zero or more <b>Categories</b>.
        </p>

        <h1>Categories</h1>
        <p className={styles.paragraph}>
            <b>Categories</b> are also user-generated and are used to describe <b>Things</b>.&nbsp;
            <u>Fruit</u> is a <b>Category</b>.&nbsp;
            <u>Movie</u> is a <b>Category</b>.&nbsp;
            <u>Planet</u> is a <b>Category</b>.
            In a single <b>Category</b>, a user can list all of the <b>Things</b> belonging to it, in the order of their ELO scores.
            THIS is how the quality of a <b>Thing</b> is derived!
        </p>

        <h1>Polls</h1>
        <p className={styles.paragraph}>
            A user can begin a series of polls in one of two ways.
            The first is by selecting a specific <b>Category</b> and starting a poll for it.
            The second is by starting a poll in a random <b>Category</b>, which can be done by tapping the button below.
        </p>
        <div className={styles.row}>
            <div className={styles.button}>
                <b>Begin Poll</b>
            </div>
        </div>
    </>
);

export default About;