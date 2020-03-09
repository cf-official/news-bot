let Discord = require('discord.js')
let meta = require('url-metadata');
let Parser = require('rss-parser');
let fs = require('fs');
let dump = require('./dump.json');
let lastUpdate = dump.lastUpdate !== undefined ? dump.lastUpdate : {};
let items = dump.items !== undefined ? dump.items : {};
let names = dump.names !== undefined ? dump.names : [];
const channels = [ "", "673202161835966485"];
const feeds = require('./feeds.json');
let client = new Discord.Client({
  disableEveryone: true
});
let parser = new Parser();
let updates = {};
client.login("Njg0MzMwNDI0MTg4NTM0Nzk0.XmXmog.Itn2Gir6-zGmq3f0pIQ1Vzf_XlM");

client.on('ready', () => {
  client.user.setActivity(`over the news`, {
    type: "WATCHING",
    url: "https://www.bbc.co.uk/iplayer/live/bbcnews"
  })
  feeds.normal.forEach(feed => {
    items[feed.cat_short] = items[feed.cat_short] !== undefined ? items[feed.cat_short] : [];
    updates[feed.cat_short] = {
      mostRecent: {},
      addedSinceLastCheck: []
    };
    getFeed(feed);
    setInterval(() => {
      updates[feed.cat_short].addedSinceLastCheck.forEach(item => {
        items[feed.cat_short].push(item);
        names.push(item.title);
      })
      getFeed(feed);
      generateUpdate(feed)
    }, 60000 * feeds.normal_mins);
  })
  feeds.frequent.forEach(feed => {
    items[feed.cat_short] = items[feed.cat_short] !== undefined ? items[feed.cat_short] : [];
    updates[feed.cat_short] = {
      mostRecent: {},
      addedSinceLastCheck: []
    };
    checkFeed(feed);
    setInterval(() => {
      updates[feed.cat_short].addedSinceLastCheck.forEach(item => {
        items[feed.cat_short].push(item);
        names.push(item.title);
      })
      checkFeed(feed);
      generateUpdate(feed)
    }, 60000 * feeds.frequent_mins)
  })
  console.log(`Buffered approximately ${names.length} articles into the system`)
  Object.keys(lastUpdate).forEach(key => {
    console.log(`Last update for ${key}: ${lastUpdate[key].title}`)
  })
  setInterval(() => {
    let totalSeconds = (client.uptime / 1000);
    let days = Math.floor(totalSeconds / 86400);
    let hours = Math.floor(totalSeconds / 3600);
    totalSeconds %= 3600;
    let minutes = Math.floor(totalSeconds / 60);
    let seconds = totalSeconds % 60;
    process.stdout.write(`${days} days, ${hours} hours, ${minutes} minutes and ${seconds.toFixed(0)} seconds\r`)
  }, 1000)
})

client.on("message", (message) => {
  // Ignore all bots
  if (message.author.bot) return;

  // Ignore messages not starting with the prefix (in config.json)
  if (message.content.indexOf("d!") !== 0) return;

  // Our standard argument/command name definition.
  const args = message.content.slice("d!".length).trim().split(/ +/g);
  const command = args.shift().toLowerCase();
  if (command === "news") {
    let em = new Discord.MessageEmbed()
    if (updates[args[0].toLowerCase()].mostRecent) {
      em.setTitle(updates[args[0].toLowerCase()].mostRecent.title)
      em.setURL(updates[args[0].toLowerCase()].mostRecent.link)
      em.setDescription(updates[args[0].toLowerCase()].mostRecent.snippet)
      em.setFooter(`Story from ${updates[args[0].toLowerCase()].mostRecent.source} in ${updates[args[0].toLowerCase()].mostRecent.category}`)
      em.setImage(updates[args[0].toLowerCase()].mostRecent.image);
    } else if (items[args[0].toLowerCase()][0]) {
      em.setTitle(items[args[0].toLowerCase()][0].title)
      em.setURL(items[args[0].toLowerCase()][0].link)
      em.setDescription(items[args[0].toLowerCase()][0].snippet)
      em.setFooter(`Story from ${items[args[0].toLowerCase()][0].source} in ${items[args[0].toLowerCase()][0].category}`)
      em.setImage(items[args[0].toLowerCase()][0].image);
    }
    if (em) {
      message.channel.send(em)
    } else {
      message.channel.send("unable to locate any relevant news articles")
    }
  } else if (command === "dump") {
    mkdump()
    message.channel.send("Wrote dump file");
  } else if (command === "breaking") {
    let em = new Discord.MessageEmbed()
    if (updates["world"].mostRecent) {
      em.setTitle(updates["world"].mostRecent.title)
      em.setURL(updates["world"].mostRecent.link)
      em.setDescription(updates["world"].mostRecent.snippet)
      em.setFooter(`Story from ${updates["world"].mostRecent.source} in ${updates["world"].mostRecent.category}`)
      em.setImage(updates["world"].mostRecent.image);
    } else if (items["world"][0]) {
      em.setTitle(items["world"][0].title)
      em.setURL(items["world"][0].link)
      em.setDescription(items["world"][0].snippet)
      em.setFooter(`Story from ${items["world"][0].source} in ${items["world"][0].category}`)
      em.setImage(items["world"][0].image);
    }
    if (em) {
      message.channel.send(em)
    } else {
      message.channel.send("unable to locate any breaking news articles")
    }
  }
})

async function getFeed(fd) {
  (async () => {
    updates[fd.cat_short] = {
      mostRecent: {},
      addedSinceLastCheck: []
    };
    let feed = await parser.parseURL(fd.link);
    feed.items.forEach(async (item) => {
      let metadata = await meta(item.link)
      let article = new Article(fd, item, metadata)
      if (!names.includes(article.title)) {
        updates[fd.cat_short].mostRecent = article;
        updates[fd.cat_short].addedSinceLastCheck.push(article);
        names.push(article.title)
        items.mostRecent = article
      }
    });
  })();
}

async function checkFeed(fd) {
  (async () => {
    updates[fd.cat_short] = {
      mostRecent: {},
      addedSinceLastCheck: []
    };
    let feed = await parser.parseURL(fd.link);
    feed.items.forEach(async (item) => {
      let metadata = await meta(item.link)
      let article = new Article(fd, item, metadata)
      if (!names.includes(article.title)) {
        updates[fd.cat_short].mostRecent = article;
        updates[fd.cat_short].addedSinceLastCheck.push(article);
        names.push(article.title)
        items.mostRecent = article
      }
    });
  })();
}

async function generateUpdate(feed) {
  return new Promise(async (resolve, reject) => {
    const articles = items[feed.cat_short][(items[feed.cat_short].length - 1)];
    if (lastUpdate[feed.cat_short].title !== articles.title) {
      channels.forEach(async (channel) => {
        let c = await client.channels.fetch(channel);
	if(c!==undefined){
	        lastUpdate[feed.cat_short] = articles;
	        let em = new Discord.MessageEmbed()
	          .setTitle(articles.title)
	          .setURL(articles.link)
	          .setDescription(articles.snippet)
	          .setFooter(`Story from ${articles.source} in ${articles.category} | Feed updated at ${new Date().toUTCString()}`)
	          .setImage(articles.image);
	        c.send(em)	
	}
      })
    }
  })
}

function mkdump() {
  feeds.normal.forEach(feed => {
    updates[feed.cat_short].addedSinceLastCheck.forEach(item => {
      items[feed.cat_short].push(item);
      names.push(item.title);
    })
  })
  feeds.frequent.forEach(feed => {
    updates[feed.cat_short].addedSinceLastCheck.forEach(item => {
      items[feed.cat_short].push(item);
      names.push(item.title);
    })
  })
  fs.writeFileSync('./dump.json', JSON.stringify({
    items,
    names,
    lastUpdate,
    dumpTime: new Date().toUTCString()
  }));
}


process.on('SIGINT', () => {
  exit();
})

process.on('SIGTERM', () => {
  exit();
})


function exit() {
  mkdump()
  console.log("\ndump file updated, exiting");
  process.exit()
}


class Article {
  constructor(source, data, metadata) {
    this.title = data.title;
    this.snippet = data.contentSnippet
    this.source = source.source
    this.category = source.category
    this.image = metadata["og:image"]
    if (source.source === "Reddit") {
      this.link = data.link
    } else if (source.source === "Reddit Live") {
      this.link = data.content.split("<a href=\"")[1].split("\">")[0];
    } else {
      this.link = data.link;
    }
    if (this.image[0] + this.image[1] === "//") {
      this.image = "http:" + this.image
    }
    //fs.writeFileSync(`./articles/${source.cat_short}-article-${this.title}.json`, JSON.stringify({source, data, metadata}))
  }
  link;
  title;
  snippet;
  source;
  category;
  image;
}

//article selections prepared according to the following guidelines stated here:
//https://discordapp.com/channels/275377268728135680/322096051894878238/322535287618142209