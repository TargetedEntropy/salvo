import React from 'react';
import { Material } from '../types';

interface MaterialsListProps {
  materials: Material[];
  totalValue: number;
}

const MaterialsList: React.FC<MaterialsListProps> = ({ materials, totalValue }) => {
  if (materials.length === 0) {
    return null;
  }

  const formatISK = (value: number): string => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(value);
  };

  return (
    <div className="card">
      <h2 className="text-2xl font-bold mb-4 text-eve-accent">Reprocessed Materials</h2>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-gray-700">
              <th className="text-left py-2 px-3">Material</th>
              <th className="text-right py-2 px-3">Quantity</th>
              <th className="text-right py-2 px-3">Unit Price</th>
              <th className="text-right py-2 px-3">Total Value</th>
            </tr>
          </thead>
          <tbody>
            {materials.map((material) => (
              <tr
                key={material.type_id}
                className="border-b border-gray-800 hover:bg-eve-gray transition-colors"
              >
                <td className="py-3 px-3 font-medium">{material.name}</td>
                <td className="py-3 px-3 text-right text-gray-300">
                  {material.quantity.toLocaleString()}
                </td>
                <td className="py-3 px-3 text-right text-gray-300">
                  {formatISK(material.unit_price)} ISK
                </td>
                <td className="py-3 px-3 text-right text-eve-gold font-semibold">
                  {formatISK(material.total_value)} ISK
                </td>
              </tr>
            ))}
          </tbody>
          <tfoot>
            <tr className="border-t-2 border-eve-accent">
              <td colSpan={3} className="py-3 px-3 font-bold text-right">
                Total Material Value:
              </td>
              <td className="py-3 px-3 text-right text-eve-gold font-bold text-lg">
                {formatISK(totalValue)} ISK
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  );
};

export default MaterialsList;
