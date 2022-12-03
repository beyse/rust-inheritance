# rust-inheritance
This is a small repo for me to learn the rust language. It focuses on inheritance.

## What it shows

It shows how to use inheritance in rust to build the following class hierarchy:

```mermaid
classDiagram
Car <|-- Sportscar : implements
Car <|-- Familycar : implements
Car <|-- CarImpl : implements
Car : accelerate()
Car : brake()
Car : get_speed()
```
