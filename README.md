# About

It is the rust daemon for notifications about the release of a recent episode of the anime. 

# Installation

```shell
cargo install --git https://github.com/Mephistophiles/anime_notify
```

# How it works

The program loads the config by path `XDG_CONFIG_HOME`, with the following structure:

```yaml
telegram_token: <TELEGRAM_TOKEN>
telegram_user_id: <TELEGRAM_USER_ID (get from IDBot)>
sites:
  - site: <SITE>
    release_days:
      Monday:
        - name: <ANIME_NAME>
          url: <ANIME_URL>
          new_episode_selector: |
                <CSS-SELECTOR>
      Tuesday:
        - name: <ANIME_NAME_2>
          url: <ANIME_URL_2>
          new_episode_selector: |
                <CSS-SELECTOR_2>
```

Program iterates by sites, followed by config for today, and tried any of kind selectors. It selector found the object - the program sends telegram notify to `<TELEGRAM_USER_ID>`.
