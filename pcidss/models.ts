import { Model, Sequelize } from "sequelize";

/**
 * @description Account model
 */
export class Account extends Model {}

/**
 * Initializes the database
 * @param db
 */
export function initDb(db: Sequelize): void {
  Account.init(
    {
      index: {
        type: DataTypes.INTEGER,
        allowNull: false,
        primaryKey: true,
      },
      name: {
        type: DataTypes.STRING,
        allowNull: false,
      },
      cardNumber: {
        type: DataTypes.STRING,
        allowNull: true,
      },
      balance: {
        type: DataTypes.STRING,
        allowNull: false,
      },
      pin: {
        type: DataTypes.INTEGER,
        allowNull: false,
      },
      expiry: {
        type: DataTypes.DATE,
        allowNull: false,
      },
      cvv: {
        type: DataTypes.INTEGER,
        allowNull: false,
      },
    },
    {
      sequelize: db,
      indexes: [
        {
          fields: ["index"],
        },
      ],
    }
  );
}
