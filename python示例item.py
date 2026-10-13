from dataclasses import dataclass

class BaseItem:
    pass

class X:
    pass

@dataclass
class Item: #(BaseItem, X):
    text: str
    done: bool = False

    def __str__(self):
        return self.text

    def say(self, greeting):
        print(greeting + " " + self.text)

    def forSaving(self) -> str:
        return f'{self.done} {self.text}'

    @staticmethod
    def fromString(line: str):
        done, text = line.split(' ', 1)
        return Item(text, done=='True')
    # @property
    # def x(self):
    #     return "hi"

    # @x.setter
    # def x(self, value):
    #     self.text = value