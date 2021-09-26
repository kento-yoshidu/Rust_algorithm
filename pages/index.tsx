import Head from 'next/head'
import Link from "next/link"

import { client } from "../libs/client"
import { Date } from "../libs/dateFormat"

import Header from "../src/components/Header"

const Styles = require("../styles/index.module.scss")

const Index = ({blog, newsList}) => {
  return (
    <div>
      <Head>
        <title>NextJS microCMS Site</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
        <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Philosopher" />
      </Head>

      <div className={Styles.bigBg}>
        <Header />

        <div className={Styles.hero}>
          <h2 className={Styles.pageTitle}>We'll Make You're Day</h2>
          <p>酒場で、指揮者が歯医者を無視する気がした。</p>
          <Link
            href="/"
          >
            <a className={Styles.button}>
              メニューを見る
            </a>
          </Link>
        </div>
      </div>

      <main className={Styles.main}>

        <section className={Styles.newsSection}>

          <div className={Styles.titleWrapper}>
            <h2 className={Styles.title}>News</h2>
          </div>

          <div className={Styles.wrapper}>

            <article>
              <header className={Styles.postInfo}>
                <h2 className={Styles.postTitle}>{newsList[0].title}</h2>
                <p className={Styles.postDate}><Date dateString={newsList[0].createdAt} /></p>
              </header>

              <div
                className={Styles.body}
                dangerouslySetInnerHTML={{
                  __html: `${newsList[0].body}`,
                }}
              ></div>
            </article>

            <aside>
              <h3 className={Styles.title}>News一覧</h3>
              <ul className={Styles.subMenu}>
                {newsList.map((news) => {
                  if (newsList[0].id !== news.id) {
                    return (
                      <li key={news.id}>
                        <Link href={`/news/${news.id}`}>
                          <a>
                            {news.title}
                          </a>
                        </Link>
                      </li> 
                    )
                  }
                })}
              </ul>
            </aside>
          </div>
        </section>

        <section className={Styles.blogSection}>

          <div className={Styles.titleWrapper}>
            <h2 className={Styles.title}>Blog</h2>
          </div>

          <div className={Styles.wrapper}>
            {blog.map((article) => (
              <h2 key={article.id}>
                {article.title}
              </h2> 
            ))}
          </div>
        </section>


      </main>
    </div>
  )
}

export default Index

export const getStaticProps = async () => {
  const blogData = await client.get<ResponseType>({
    endpoint: "blog",
  })

  const newsData = await client.get<ResponseType>({
    endpoint: "news",
    queries: {
      filters: "flag[equals]true",
    }
  })

  return {
    props: {
      blog: blogData.contents,
      newsList: newsData.contents
    }
  }
}