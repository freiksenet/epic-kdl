# epic-kdl

KDL format, list printer and potentially list builder for Epic: Armageddon armies.

Very early stage.

## Practicalities

run npm install to get prettier plugins for KDL and jinja templates. Otherwise Rust.

## TODO / Questions

- [ ] How to write units

```
formation "Bike" {
  cost 200
  units {
    unit 5 "Bike" "Attack Bike"
  }
  upgrades {
    upgrade "Commander" max=1
  }
}
```

or

```
formation "Bike" {
  cost 200
  unit 5 "Bike" "Attack Bike"
  upgrade "Commander" max=1
}
```

- [ ] How to write upgrades.
  - [ ] Enumerate kinds of upgrades - is it only replace and add?
  - [ ] Some lists have native "only one upgrade per formation", is it worth special ruling that?
  - [ ] Some lists have inline upgrades (Orks) - does it make sense to separate them?
  - [ ] Some lists have formation variants ('uge in orks etc)
  - [ ] Transports - inclined to make it a free upgrade or smth like that
- [ ] Where initiative rating goes
- [ ] List constrainst - currently inline with formation-groups, but can be a separate thing
- [ ] TODO - Special rules
- [ ] TODO -
- [ ] TODO - importing from one file to another (eg forces or special rules)

## Plans

- List printer with similar looks as NetEA lists
  - wahapedia-like tooltip inspector / hyperlinker for forces, units and special rules
  - wahapedia-like navigator / contents
  - printable to pdf out of the box (so right click -> print should just work)
- List builder similar to Agen/brumbaer

# LEGAL

Contains re-adapted lists from NetEA.

This project is completely unofficial and in no way endorsed by Games Workshop Limited.

GW, Games Workshop, Citadel, Black Library, Forge World, Warhammer, the Twin-tailed Comet logo, Warhammer 40,000, the ‘Aquila’ Double-headed Eagle logo, Space Marine, 40K, 40,000, Warhammer Age of Sigmar, Battletome, Stormcast Eternals, White Dwarf, Blood Bowl, Necromunda, Space Hulk, Battlefleet Gothic, Dreadfleet, Mordheim, Inquisitor, Warmaster, Epic, Gorkamorka, and all associated logos, illustrations, images, names, creatures, races, vehicles, locations, weapons, characters, and the distinctive likenesses thereof, are either ® or TM, and/or © Games Workshop Limited, variably registered around the world. All Rights Reserved.
