"""Functions to manage a users shopping cart items."""

from typing import Iterable, TypeAlias
from functools import reduce

Recipe: TypeAlias = dict[str, int]
Cart = Recipe

def _add(cart: Cart, item_to_add: str) -> Cart:
    """
    Utility function for reducer to add to recipe
    """
    cart[item_to_add] = cart.get(item_to_add, 0) + 1
    return cart

def add_item(current_cart: Cart, items_to_add: Iterable[str]) -> Cart:
    """Add items to shopping cart.

    :param current_cart: dict - the current shopping cart.
    :param items_to_add: iterable - items to add to the cart.
    :return: dict - the updated user cart dictionary.
    """
    return reduce(_add, items_to_add, current_cart) 

def read_notes(notes: Iterable[str]) -> Cart:
    """Create user cart from an iterable notes entry.

    :param notes: iterable of items to add to cart.
    :return: dict - a user shopping cart dictionary.
    """
    return reduce(_add, notes, {})

Ideas: TypeAlias = dict[str, Recipe]
def update_recipes(ideas: Ideas, recipe_updates: Iterable[tuple[str, Recipe]]) -> Ideas:
    """Update the recipe ideas dictionary.

    :param ideas: dict - The "recipe ideas" dict.
    :param recipe_updates: iterable -  with updates for the ideas section.
    :return: dict - updated "recipe ideas" dict.
    """
    return ideas | dict(recipe_updates)

def sort_entries(cart: Cart) -> Cart:
    """Sort a users shopping cart in alphabetically order.

    :param cart: dict - a users shopping cart dictionary.
    :return: dict - users shopping cart sorted in alphabetical order.
    """ 
    return dict(sorted(cart.items()))

"""Aisle name, needs fridge"""
Aisle: TypeAlias = tuple[str, bool]
"""ingredient, aisle"""
AisleMapping: TypeAlias = list[str, Aisle]
""", Aisle name, needs fridge"""
AisleFullfillmentInfo: TypeAlias = list[int, str, bool]
AisleFullfillmentCart: TypeAlias = dict[str, AisleFullfillmentInfo]

def send_to_store(cart: Cart, aisle_mapping: AisleMapping) -> AisleFullfillmentCart:
    """Combine users order to aisle and refrigeration information.

    :param cart: dict - users shopping cart dictionary.
    :param aisle_mapping: dict - aisle and refrigeration information dictionary.
    :return: dict - fulfillment dictionary ready to send to store.
    """
    return dict(reversed(sorted(
        (
            [ingredient, [cart.get(ingredient, 0), *aisle]]
            for ingredient, aisle in aisle_mapping.items()
            if cart.get(ingredient, 0) > 0
        ),
        key=lambda item: item[0]
    )))

def update_store_inventory(
    fulfillment_cart: AisleFullfillmentCart, 
    store_inventory: AisleFullfillmentCart
) -> AisleFullfillmentCart:
    """Update store inventory levels with user order.

    :param fulfillment cart: dict - fulfillment cart to send to store.
    :param store_inventory: dict - store available inventory
    :return: dict - store_inventory updated.
    """
    return {ingredient:
        [
            (quantity - fulfillment_cart.get(ingredient, (0,))[0]
                if quantity - fulfillment_cart.get(ingredient, (0,))[0] > 0
                else 'Out of Stock'
            ),
            aisle,
            refrigeration
        ]
        for ingredient, (quantity, aisle, refrigeration) in store_inventory.items()
    }





















