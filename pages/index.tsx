import Head from 'next/head'
import styles from '../styles/index.module.scss'

import Test from "../src/components/Test"

const Index = () => {
  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>

        <Test
          message="test component"
        />
      
      </main>
    </div>
  )
}

export default Index