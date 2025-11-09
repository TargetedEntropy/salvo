import React, { useState } from 'react';
import { SalvageItem } from '../types';

interface SalvageInputProps {
  salvageItems: SalvageItem[];
  onAddItem: (item: SalvageItem) => void;
  onRemoveItem: (index: number) => void;
  onClear: () => void;
}

const COMMON_SALVAGE = [
  'Tripped Power Circuit',
  'Charred Micro Circuit',
  'Fried Interface Circuit',
  'Contaminated Nanite Compound',
  'Armor Plates',
  'Burned Logic Circuit',
];

const SalvageInput: React.FC<SalvageInputProps> = ({
  salvageItems,
  onAddItem,
  onRemoveItem,
  onClear,
}) => {
  const [name, setName] = useState('');
  const [quantity, setQuantity] = useState('');

  const handleAdd = () => {
    if (name.trim() && quantity && parseInt(quantity) > 0) {
      onAddItem({
        name: name.trim(),
        quantity: parseInt(quantity),
      });
      setName('');
      setQuantity('');
    }
  };

  const handleQuickAdd = (itemName: string) => {
    setName(itemName);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleAdd();
    }
  };

  return (
    <div className="card">
      <h2 className="text-2xl font-bold mb-4 text-eve-accent">Salvage Input</h2>

      <div className="mb-4">
        <label className="label">Item Name</label>
        <input
          type="text"
          className="input w-full"
          value={name}
          onChange={(e) => setName(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Enter salvage item name"
          list="salvage-items"
        />
        <datalist id="salvage-items">
          {COMMON_SALVAGE.map((item) => (
            <option key={item} value={item} />
          ))}
        </datalist>
      </div>

      <div className="mb-4">
        <label className="label">Quantity</label>
        <input
          type="number"
          className="input w-full"
          value={quantity}
          onChange={(e) => setQuantity(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Enter quantity"
          min="1"
        />
      </div>

      <div className="flex gap-2 mb-4">
        <button onClick={handleAdd} className="btn-primary flex-1">
          Add Item
        </button>
        {salvageItems.length > 0 && (
          <button onClick={onClear} className="btn-secondary">
            Clear All
          </button>
        )}
      </div>

      <div className="mb-4">
        <label className="label">Quick Add</label>
        <div className="flex flex-wrap gap-2">
          {COMMON_SALVAGE.map((item) => (
            <button
              key={item}
              onClick={() => handleQuickAdd(item)}
              className="text-xs bg-gray-700 hover:bg-gray-600 text-gray-100 px-2 py-1 rounded transition-colors"
            >
              {item}
            </button>
          ))}
        </div>
      </div>

      {salvageItems.length > 0 && (
        <div>
          <h3 className="text-lg font-semibold mb-2">Current Items</h3>
          <div className="space-y-2">
            {salvageItems.map((item, index) => (
              <div
                key={index}
                className="flex justify-between items-center bg-eve-gray p-3 rounded"
              >
                <div>
                  <span className="font-medium">{item.name}</span>
                  <span className="text-gray-400 ml-2">×{item.quantity}</span>
                </div>
                <button
                  onClick={() => onRemoveItem(index)}
                  className="text-red-400 hover:text-red-300 transition-colors"
                >
                  Remove
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SalvageInput;
