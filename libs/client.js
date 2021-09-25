import { createClient } from "microcms-js-sdk"

export const client = createClient({
	serviceDomain: "jamsample",
	apiKey: process.env.API_KEY
})